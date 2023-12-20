import itertools
import os
import subprocess
from concurrent.futures import ProcessPoolExecutor

RESULTS_PATH = "./analysis/data/"

TRANSMISSION_RATE_RANGE = [x / 100 for x in range(2, 22, 2)]  # 0.1
MORTALITY_RATE_RANGE = [x / 100 for x in range(1, 11)]  # 0.05
INFECTION_PERIOD_RANGE = range(1, 8)  # 3
RUNS = range(0, 10)

TRANSMISSION_RATE_EXPERIMENT = [
    [
        [
            "-t",
            f"{x:.2f}",
            "-i",
            "3",
            "-m",
            "0.05",
            "--data-output-path",
            RESULTS_PATH + f"transmission/tra{x:.2f}_run{run}.txt",
        ]
        for x in TRANSMISSION_RATE_RANGE
        for run in RUNS
    ]
]
MORTALITY_RATE_EXPERIMENT = [
    [
        [
            "-t",
            "0.1",
            "-i",
            "3",
            "-m",
            f"{x:.2f}",
            "--data-output-path",
            RESULTS_PATH + f"mortality/mor{x:.2f}_run{run}.txt",
        ]
        for x in MORTALITY_RATE_RANGE
        for run in RUNS
    ]
]
INFECTION_PERIOD_EXPERIMENT = [
    [
        [
            "-t",
            "0.1",
            "-i",
            str(x),
            "-m",
            "0.05",
            "--data-output-path",
            RESULTS_PATH + f"period/per{x}_run{run}.txt",
        ]
        for x in INFECTION_PERIOD_RANGE
        for run in RUNS
    ]
]


for name in ["transmission", "mortality", "period"]:
    os.makedirs(RESULTS_PATH + name, exist_ok=True)


def run_subprocess(arguments_list: list[str]):
    print(f"Starting run with arguments: {arguments_list}")
    subprocess.run(
        ["./target/release/pandemic_dynamics_sim", "-o", "100"] + arguments_list,
        capture_output=True,
        text=True,
    )


all_args = list(
    itertools.chain.from_iterable(
        (
            TRANSMISSION_RATE_EXPERIMENT
            + MORTALITY_RATE_EXPERIMENT
            + INFECTION_PERIOD_EXPERIMENT
        )
    )
)

with ProcessPoolExecutor(max_workers=len(all_args)) as executor:
    for result in executor.map(
        run_subprocess,
        all_args,
    ):
        print(result)
