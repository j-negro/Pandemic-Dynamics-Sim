import os
import subprocess
from concurrent.futures import ProcessPoolExecutor

RESULTS_PATH = "./analysis/data/"

TRANSMISSION_RATE_RANGE = [x / 100 for x in range(5, 55, 5)]  # 0.4
MORTALITY_RATE_RANGE = [x / 100 for x in range(2, 22, 2)]  # 0.10
INFECTION_PERIOD_RANGE = range(2, 8)  # 3

TRANSMISSION_RATE_EXPERIMENT = [
    [
        [
            "-t",
            str(x),
            "-i",
            "3",
            "-m",
            "0.1",
            "--data-output-path",
            RESULTS_PATH + f"transmission/tra{x}_run{run}.txt",
        ]
        for x in TRANSMISSION_RATE_RANGE
        for run in range(0, 3)
    ]
]
MORTALITY_RATE_EXPERIMENT = [
    [
        [
            "-t",
            "0.25",
            "-i",
            "3",
            "-m",
            str(x),
            "--data-output-path",
            RESULTS_PATH + f"mortality/mor{x}_run{run}.txt",
        ]
        for x in MORTALITY_RATE_RANGE
        for run in range(0, 3)
    ]
]
INFECTION_PERIOD_EXPERIMENT = [
    [
        [
            "-t",
            "0.25",
            "-i",
            str(x),
            "-m",
            "0.1",
            "--data-output-path",
            RESULTS_PATH + f"period/per{x}_run{run}.txt",
        ]
        for x in INFECTION_PERIOD_RANGE
        for run in range(0, 3)
    ]
]


for name in ["transmission", "mortality", "period"]:
    os.makedirs(RESULTS_PATH + name, exist_ok=True)


subprocess.run(
    "cargo build -r",
)


def run_subprocess(arguments_list: list[str]):
    print(f"Starting run with arguments: {arguments_list}")
    subprocess.run(
        ["./target/release/pandemic_dynamics_sim", "-o", "100"] + arguments_list,
        capture_output=True,
        text=True,
    )


all_args = (
    TRANSMISSION_RATE_EXPERIMENT
    + MORTALITY_RATE_EXPERIMENT
    + INFECTION_PERIOD_EXPERIMENT
)

with ProcessPoolExecutor(max_workers=len(all_args)) as executor:
    for result in executor.map(
        run_subprocess,
        all_args,
    ):
        print(result)
