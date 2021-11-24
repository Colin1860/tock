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

## New Task from ISR

- From _NEW_TASK_FROM_ISR_START_ in chips/nrf5x/src/gpio.rs:589
- To _NEW_TASK_FROM_ISR_STOP_ in kernel/src/process_standard.rs.:1045
- Took in average: 4450 ticks

## Notes:

- I'm writing once after each measurement, when the timer is already stopped, to the debug output via Segger RTT, which comes with considerable overhead as i experienced. Even though the actual measured code is free from debug output, i can't rule out that the measured value is not impacted by the console output.

## To come shortly:

### Cooperative Task Switch
