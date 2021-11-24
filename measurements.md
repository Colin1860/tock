# Measurements

## ISR2-Latency

- From _ISR2_LATENCY_START_ in arch/cortex-m/src/lib.rs:137
- To _ISR2_LATENCY_STOP_ in chips/nrf5x/src/gpio.rs:575
- Took in average: 215 ticks

## ISR2-Exit

- From _ISR2_EXIT_START_ in chips/nrf5x/src/gpio.rs:589
- To _ISR2_EXIT_STOP_ in arch/cortex-m/src/lib.rs:329
- Took in average: 1250 ticks
- Note: Don't know why this takes so much longer than the entry part of the isr2 yet

## Preemptive Task Switch

- From _PREEMPTIVE_SWITCH_START_ in kernel/src/scheduler/priority.rs:51
- To _PREEMPTIVE_SWITCH_STOP_ in kernel/src/process_standard.rs.:1045
- Took in average: 4250 ticks

## To come shortly:

### Cooperative Task Switch

### New Task from ISR
