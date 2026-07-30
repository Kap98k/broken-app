#!/usr/bin/env python3
"""
Генерирует графики из JSON-отчётов criterion.
Требуется: pip install matplotlib
Запуск:
  python3 scripts/plot_benches.py                                    # обычные графики из target/criterion
  python3 scripts/plot_benches.py --criterion-dir artifacts/baseline  # графики из произвольной папки
  python3 scripts/plot_benches.py --compare <baseline.csv>            # сравнительные графики + дельта
  python3 scripts/plot_benches.py --criterion-dir artifacts/baseline --compare artifacts/baseline/all_benchmarks.csv
"""
import json
import sys
from pathlib import Path
from collections import defaultdict

OUT_DIR = Path("artifacts/plots")
COMPARE_DIR = Path("artifacts/compare")


def load_all_reports(criterion_dir: Path) -> dict[str, list[dict]]:
    """Собирает все бенчмарки: {group_name: [{name, mean_ns, std_ns}, ...]}"""
    groups = defaultdict(list)
    for bench_json in sorted(criterion_dir.glob("**/new/estimates.json")):
        with open(bench_json) as f:
            data = json.load(f)
        mean_ns = data["mean"]["point_estimate"]
        std_ns = data["std_dev"]["point_estimate"]
        rel = bench_json.parent.parent.relative_to(criterion_dir)
        group = str(rel.parent) if rel.parent != Path(".") else str(rel)
        name = str(rel)
        groups[group].append({
            "name": name,
            "mean_ns": mean_ns,
            "std_ns": std_ns,
            "mean_us": mean_ns / 1_000,
            "mean_ms": mean_ns / 1_000_000,
        })
    return dict(groups)


def load_csv(path: str) -> dict[str, dict]:
    """Загружает CSV в {benchmark_name: {mean_ns, std_ns, mean_us, mean_ms}}"""
    result = {}
    with open(path) as f:
        header = f.readline().strip().split(",")
        for line in f:
            parts = line.strip().split(",")
            if len(parts) >= 6:
                name = parts[1]
                result[name] = {
                    "mean_ns": float(parts[2]),
                    "std_ns": float(parts[3]),
                    "mean_us": float(parts[4]),
                    "mean_ms": float(parts[5]),
                }
    return result


def plot_groups(groups: dict):
    import matplotlib.pyplot as plt
    import numpy as np

    OUT_DIR.mkdir(parents=True, exist_ok=True)

    for group_name, benches in sorted(groups.items()):
        if not benches:
            continue
        names = [b["name"] for b in benches]
        means = [b["mean_us"] for b in benches]
        stds = [b["std_ns"] / 1_000 for b in benches]

        fig, ax = plt.subplots(figsize=(10, 5))
        x = np.arange(len(names))
        bars = ax.bar(x, means, yerr=stds, capsize=5, color="#4C72B0", edgecolor="#2C3E50")
        ax.set_xticks(x)
        ax.set_xticklabels(names, rotation=30, ha="right", fontsize=9)
        ax.set_ylabel("Время (µs)")
        ax.set_title(f"Бенчмарк: {group_name}")
        ax.grid(axis="y", alpha=0.3)

        for bar, mean in zip(bars, means):
            ax.text(bar.get_x() + bar.get_width() / 2, bar.get_height(),
                    f"{mean:.1f} µs", ha="center", va="bottom", fontsize=8)

        fig.tight_layout()
        safe_name = group_name.replace("/", "_")
        path = OUT_DIR / f"{safe_name}.png"
        fig.savefig(path, dpi=150)
        plt.close(fig)
        print(f"  ✓ {path}")


def export_comparison_csv(current: dict[str, list[dict]], baseline: dict[str, dict]):
    """Экспортирует CSV сравнения в artifacts/compare/."""
    COMPARE_DIR.mkdir(parents=True, exist_ok=True)
    csv_path = COMPARE_DIR / "comparison.csv"
    with open(csv_path, "w") as f:
        f.write("group,benchmark,baseline_ns,current_ns,baseline_us,current_us,delta_pct\n")
        for group_name, benches in sorted(current.items()):
            for b in benches:
                bl = baseline.get(b["name"], {})
                base_ns = bl.get("mean_ns", 0)
                base_us = bl.get("mean_us", 0)
                cur_ns = b["mean_ns"]
                cur_us = b["mean_us"]
                delta = ((cur_us - base_us) / base_us * 100) if base_us > 0 else 0.0
                f.write(f"{group_name},{b['name']},{base_ns:.1f},{cur_ns:.1f},"
                        f"{base_us:.1f},{cur_us:.1f},{delta:+.2f}\n")
    print(f"  ✓ CSV сравнения: {csv_path}")


def plot_comparison(current: dict[str, list[dict]], baseline: dict[str, dict]):
    """Строит сравнительные grouped bar charts: baseline vs current + дельта."""
    import matplotlib.pyplot as plt
    import numpy as np

    COMPARE_DIR.mkdir(parents=True, exist_ok=True)

    for group_name, benches in sorted(current.items()):
        if not benches:
            continue

        names = [b["name"] for b in benches]
        cur_means = [b["mean_us"] for b in benches]
        cur_stds = [b["std_ns"] / 1_000 for b in benches]
        base_means = [baseline.get(n, {}).get("mean_us", 0) for n in names]
        base_stds = [baseline.get(n, {}).get("std_ns", 0) / 1_000 for n in names]

        deltas = []
        for c, b in zip(cur_means, base_means):
            if b > 0:
                deltas.append(((c - b) / b) * 100)
            else:
                deltas.append(0.0)

        fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(12, 8),
                                       gridspec_kw={"height_ratios": [3, 1]})
        fig.suptitle(f"Сравнение: {group_name}", fontsize=14)

        x = np.arange(len(names))
        w = 0.35

        bars1 = ax1.bar(x - w / 2, base_means, w, yerr=base_stds, capsize=3,
                        label="Baseline", color="#95A5A6", edgecolor="#7F8C8D")
        bars2 = ax1.bar(x + w / 2, cur_means, w, yerr=cur_stds, capsize=3,
                        label="Current", color="#4C72B0", edgecolor="#2C3E50")
        ax1.set_xticks(x)
        ax1.set_xticklabels(names, rotation=30, ha="right", fontsize=9)
        ax1.set_ylabel("Время (µs)")
        ax1.legend(fontsize=10)
        ax1.grid(axis="y", alpha=0.3)

        for bar, mean in zip(bars1, base_means):
            ax1.text(bar.get_x() + bar.get_width() / 2, bar.get_height(),
                     f"{mean:.1f}", ha="center", va="bottom", fontsize=7, color="#555")
        for bar, mean in zip(bars2, cur_means):
            ax1.text(bar.get_x() + bar.get_width() / 2, bar.get_height(),
                     f"{mean:.1f}", ha="center", va="bottom", fontsize=7, color="#2C3E50")

        colors = ["#E74C3C" if d > 0 else "#2ECC71" for d in deltas]
        ax2.bar(x, deltas, w * 1.5, color=colors, edgecolor="#333", alpha=0.8)
        ax2.axhline(y=0, color="black", linewidth=0.8)
        ax2.set_xticks(x)
        ax2.set_xticklabels(names, rotation=30, ha="right", fontsize=9)
        ax2.set_ylabel("Δ (%)")
        ax2.set_xlabel("Бенчмарк")
        ax2.grid(axis="y", alpha=0.3)

        for i, d in enumerate(deltas):
            label = f"{d:+.1f}%"
            va = "bottom" if d >= 0 else "top"
            ax2.text(i, d, label, ha="center", va=va, fontsize=8, fontweight="bold")

        fig.tight_layout()
        safe_name = group_name.replace("/", "_")
        path = COMPARE_DIR / f"compare_{safe_name}.png"
        fig.savefig(path, dpi=150)
        plt.close(fig)
        print(f"  ✓ {path}")


def export_csv(groups: dict, csv_path: Path | None = None):
    if csv_path is None:
        csv_path = OUT_DIR / "all_benchmarks.csv"
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    with open(csv_path, "w") as f:
        f.write("group,benchmark,mean_ns,std_ns,mean_us,mean_ms\n")
        for group_name, benches in sorted(groups.items()):
            for b in benches:
                f.write(f"{group_name},{b['name']},{b['mean_ns']:.1f},"
                        f"{b['std_ns']:.1f},{b['mean_us']:.1f},{b['mean_ms']:.3f}\n")
    print(f"  ✓ CSV: {csv_path}")


def print_summary_table(current: dict[str, list[dict]], baseline: dict[str, dict] | None = None):
    """Печатает сводную таблицу в консоль."""
    print()
    header = f"{'Group':<25} {'Benchmark':<30} {'Mean (µs)':>12} {'Std (µs)':>12}"
    if baseline:
        header += f" {'Baseline (µs)':>14} {'Delta %':>10}"
    print(header)
    print("-" * len(header))

    for group_name, benches in sorted(current.items()):
        for b in benches:
            line = f"{group_name:<25} {b['name']:<30} {b['mean_us']:>12.1f} {b['std_ns']/1000:>12.1f}"
            if baseline:
                bl = baseline.get(b["name"], {})
                base_us = bl.get("mean_us", 0)
                delta = ((b["mean_us"] - base_us) / base_us * 100) if base_us > 0 else 0
                line += f" {base_us:>14.1f} {delta:>+9.1f}%"
            print(line)
    print()


def main():
    # Парсим --criterion-dir
    criterion_dir = Path("target/criterion")
    if "--criterion-dir" in sys.argv:
        idx = sys.argv.index("--criterion-dir")
        criterion_dir = Path(sys.argv[idx + 1])

    groups = load_all_reports(criterion_dir)
    if not groups:
        print(f"❌ Не найдены criterion-отчёты в {criterion_dir}. Запустите сначала: cargo bench --bench criterion")
        sys.exit(1)

    # Определяем путь для CSV
    csv_path = OUT_DIR / "all_benchmarks.csv"
    if "--csv" in sys.argv:
        idx = sys.argv.index("--csv")
        csv_path = Path(sys.argv[idx + 1])

    export_csv(groups, csv_path)

    if "--compare" in sys.argv:
        idx = sys.argv.index("--compare")
        baseline_csv = sys.argv[idx + 1]
        baseline = load_csv(baseline_csv)
        print_summary_table(groups, baseline)
        export_comparison_csv(groups, baseline)
        plot_comparison(groups, baseline)
    else:
        print_summary_table(groups)
        plot_groups(groups)


if __name__ == "__main__":
    main()