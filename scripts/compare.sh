#!/usr/bin/env bash
set -euo pipefail

# ═══════════════════════════════════════════════════════════════════════════════
#  Полный пайплайн: бенчмарки → CSV → сравнительные графики
#  Использование:
#    ./scripts/compare.sh              # запустить бенчмарки и построить графики
#    ./scripts/compare.sh --baseline   # сгенерировать CSV из artifacts/baseline/ (без запуска бенчмарков)
#    ./scripts/compare.sh --compare    # запустить бенчмарки, сравнить с artifacts/baseline/
# ═══════════════════════════════════════════════════════════════════════════════

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ARTIFACTS="$PROJECT_DIR/artifacts"
BASELINE_DIR="$ARTIFACTS/baseline"
BASELINE_CSV="$BASELINE_DIR/all_benchmarks.csv"
CURRENT_CSV="$ARTIFACTS/plots/all_benchmarks.csv"

run_bench() {
    echo "🚀 Запуск criterion-бенчмарков..."
    cd "$PROJECT_DIR"
    cargo bench --bench criterion 2>&1 | tee "$ARTIFACTS/bench_output.txt"
}

plot() {
    echo "📊 Генерация графиков и CSV..."
    python3 "$SCRIPT_DIR/plot_benches.py" "$@"
}

save_baseline() {
    echo "📦 Генерация baseline CSV из $BASELINE_DIR..."
    mkdir -p "$BASELINE_DIR"

    if [ ! -d "$BASELINE_DIR" ]; then
        echo "❌ Папка $BASELINE_DIR не найдена"
        exit 1
    fi

    # Генерируем CSV из criterion-отчётов в artifacts/baseline/
    python3 "$SCRIPT_DIR/plot_benches.py" \
        --criterion-dir "$BASELINE_DIR" \
        --csv "$BASELINE_CSV"

    echo "  ✓ Baseline CSV: $BASELINE_CSV"
}

compare_with_baseline() {
    if [ ! -f "$BASELINE_CSV" ]; then
        echo "⚠ Baseline CSV не найден: $BASELINE_CSV"
        echo "  Генерирую из $BASELINE_DIR..."
        save_baseline
    fi

    echo "📊 Сравнение с baseline..."
    python3 "$SCRIPT_DIR/plot_benches.py" \
        --criterion-dir "$PROJECT_DIR/target/criterion" \
        --compare "$BASELINE_CSV"
}

case "${1:-}" in
    --baseline)
        save_baseline
        ;;
    --compare)
        #run_bench
        compare_with_baseline
        ;;
    *)
        run_bench
        plot
        ;;
esac

echo ""
echo "✅ Готово! Артефакты:"
echo "   Лог бенчмарков:  $ARTIFACTS/bench_output.txt"
echo "   Графики:          $ARTIFACTS/plots/*.png"
echo "   CSV:              $CURRENT_CSV"
echo "   HTML-отчёт:       target/criterion/report/index.html"
echo ""
echo "📂 Сравнение с baseline:"
echo "   CSV сравнения:    $ARTIFACTS/compare/comparison.csv"
echo "   Графики:          $ARTIFACTS/compare/compare_*.png"
