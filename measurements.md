# Measurements

## Sources

- <https://erika.tuxfamily.org/wiki/index.php?title=Erika_Enterprise_Benchmark_for_Cortex_M4>

## Description of measured metrics

- Following the "Standardized RTOS benchmarks metrics" Jim Cooling describes in his book "Real-time operating systems - Book 1" and the benchmarks the ErikaOS team provides I'm measuring five different metrics.

- Interrupt Service Routine latency -> time elapsed from when the kernel receives an interrupt until execution of the first instruction of the ISR

- Interrupt Service Routine exit -> time elapsed from the last instruction of a ISR to the subsequent instruction of the interrupted task/process

- New task from an ISR -> time elapsed from the last instruction of an ISR to the first instruction of a new task/process

- Preemption time -> the time a high-priority task takes to preempt a running lower-priority task

- Switch without preemption -> the time elapsed switching from one task to another when using a cooperative scheduling policy

## Measuring setup

Hardware: NRF52840dk

Unit: Ticks or rather Processor Cycles

If you're interested in the actual code of the applications, you can find it here at my own fork of libtock-c: https://github.com/Colin1860/libtock-c/tree/time-measuring-support/examples -> busy and busier

- _Setup 1_: Two process loaded to board, each of them running a "while(true)" loop with a call to "delay_ms" for some milliseconds.Scheduled by the fixed priority scheduler on the kernel side
- _Setup 2_: Same as Case 1 just with the cooperative scheduler.
- _Setup 3_: One process loaded to board which registers a callback which is triggered after a button press

## ISR2-Latency

Measured:

- From _ISR2_LATENCY_START_ in arch/cortex-m/src/lib.rs:137
- To _ISR2_LATENCY_STOP_ in chips/nrf5x/src/gpio.rs:575
- Setup: 3

| OS    | MIN       | AVERAGE   | MAX       |
| ----- | --------- | --------- | --------- |
| TOCK  | 213 ticks | 215 ticks | 216 ticks |
| ERIKA | 104 ticks | 104 ticks | 104 ticks |

## ISR2-Exit

Measured:

- From _ISR2_EXIT_START_ in chips/nrf5x/src/gpio.rs:589
- To _ISR2_EXIT_STOP_ in arch/cortex-m/src/lib.rs:329
- Setup: 3

| OS    | MIN        | AVERAGE    | MAX        |
| ----- | ---------- | ---------- | ---------- |
| TOCK  | 1250 ticks | 1250 ticks | 1255 ticks |
| ERIKA | 138 ticks  | 138 ticks  | 138 ticks  |

- Note: Don't know why this takes so much longer than the entry part of the isr2 yet

## Preemptive Task Switch

Measured:

- From _PREEMPTIVE_SWITCH_START_ in kernel/src/scheduler/priority.rs:51
- To _PREEMPTIVE_SWITCH_STOP_ in kernel/src/process_standard.rs.:1045
- Setup: 1

| OS    | MIN        | AVERAGE    | MAX        |
| ----- | ---------- | ---------- | ---------- |
| TOCK  | 4248 ticks | 4250 ticks | 4256 ticks |
| ERIKA | 314 ticks  | 314 ticks  | 314 ticks  |

## New Task from ISR

Measured:

- From _NEW_TASK_FROM_ISR_START_ in chips/nrf5x/src/gpio.rs:589
- To _NEW_TASK_FROM_ISR_STOP_ in kernel/src/process_standard.rs.:1045
- Setup: 3

| OS    | MIN        | AVERAGE    | MAX        |
| ----- | ---------- | ---------- | ---------- |
| TOCK  | 4450 ticks | 4452 ticks | 4452 ticks |
| ERIKA | 345 ticks  | 345 ticks  | 345 ticks  |

## Cooperative Task Switch

Measured:

- From _COOPERATIVE_SWITCH_START_ in kernel/src/scheduler/cooperative.rs:81
- To _COOPERATIVE_SWITCH_STOP_ in kernel/src/process_standard.rs.:1045
- Setup: 2

| OS    | MIN        | AVERAGE    | MAX        |
| ----- | ---------- | ---------- | ---------- |
| TOCK  | 3910 ticks | 3910 ticks | 3910 ticks |
| ERIKA | 164 ticks  | 164 ticks  | 164 ticks  |

## Notes:

- I'm writing once after each measurement, when the timer is already stopped, to the debug output via Segger RTT, which comes with considerable overhead when the timer is not stopped as i experienced. After disabling the interrupt which is triggered when Segger RTT writes to the console, i can ensure that my debug output has no impact on the measured value.
