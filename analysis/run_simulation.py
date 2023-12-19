import os
import subprocess

RESULTS_PATH = "./analysis/data/"

DATA = [(1.2, 200), (1.8, 260), (2.4, 320), (3.0, 380)]

TRANSMISSION_RATE_RANGE = [x / 10 for x in range(1, 9)]  # 0.4
MORTALITY_RATE_RANGE = [x / 100 for x in range(1, 9)]  # 0.04
INFECTION_PERIOD_RANGE = range(2, 7)  # 3

TRANSMISSION_RATE_EXPERIMENT = [
    [
        "-t",
        str(x),
        "-i",
        "3",
        "-m",
        "0.05",
        "--data-output-path",
        RESULTS_PATH + "transmission/" + str(x) + ".txt",
    ]
    for x in TRANSMISSION_RATE_RANGE
]
MORTALITY_RATE_EXPERIMENT = [
    [
        "-t",
        "0.4",
        "-i",
        "3",
        "-m",
        str(x),
        "--data-output-path",
        RESULTS_PATH + "mortality/" + str(x) + ".txt",
    ]
    for x in TRANSMISSION_RATE_RANGE
]
INFECTION_PERIOD_EXPERIMENT = [
    [
        "-t",
        "0.4",
        "-i",
        str(x),
        "-m",
        "0.05",
        "--data-output-path",
        RESULTS_PATH + "period/" + str(x) + ".txt",
    ]
    for x in TRANSMISSION_RATE_RANGE
]


for name in ["transmission", "mortality", "period"]:
    os.makedirs(RESULTS_PATH + name, exist_ok=True)


for arguments_str in (
    TRANSMISSION_RATE_EXPERIMENT
    + MORTALITY_RATE_EXPERIMENT
    + INFECTION_PERIOD_EXPERIMENT
):
    print(f"Starting runs with arguments: {arguments_str}")
    subprocess.run(
        ["./target/release/pandemic_dynamics_sim", "-o", "100"] + arguments_str,
        capture_output=True,
        text=True,
    )
