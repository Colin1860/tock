# Measurements

## ISR2-Latency

- From "ISR2_LATENCY_START" in arch/cortex-m/src/lib.rs:137
- To "ISR2_LATENCY_STOP" in chips/nrf5x/src/gpio.rs:575
- Took in average: 215 ticks

## ISR2-Exit

- From "ISR2_EXIT_START" in chips/nrf5x/src/gpio.rs:589
- To "ISR2_EXIT_STOP" in arch/cortex-m/src/lib.rs:329
- Took in average: 1250 ticks

## Preemptive Task Switch

- From "PREEMPTIVE_SWITCH_START" in kernel/src/scheduler/priority.rs:74
- To "PREEMPTIVE_SWITCH_STOP" in kernel/src/process_standard.rs.:1045
- Took in average: 2580 ticks

## To come shortly:

### Cooperative Task Switch

### New Task from ISR
