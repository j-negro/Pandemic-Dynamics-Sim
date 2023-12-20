import os

import matplotlib.pyplot as plt
import numpy as np
from matplotlib.ticker import MultipleLocator

DIR = "./analysis/data/"

EXPERIMENTS = ["transmission", "period", "mortality"]
RUN_COUNT = 3


class InfectionStatus:
    def __init__(
        self, day: int, susceptible: int, infected: int, recovered: int, dead
    ) -> None:
        self.day = float(day)
        self.susceptible = float(susceptible)
        self.infected = float(infected)
        self.recovered = float(recovered)
        self.dead = float(dead)

    def __add__(self, other):
        if other is None:
            return self
        self.susceptible += other.susceptible
        self.infected += other.infected
        self.recovered += other.recovered
        self.dead += other.dead
        return self

    def divide_by(self, denominator: int):
        self.susceptible /= denominator
        self.infected /= denominator
        self.recovered /= denominator
        self.dead /= denominator

    def __str__(self):
        return f"Day {self.day} ; Susceptible {self.susceptible} ; Infected {self.infected} ; Recovered {self.recovered} ; Dead {self.dead}"


def add_status(
    fst: InfectionStatus | None, snd: InfectionStatus | None
) -> InfectionStatus:
    if fst is not None and snd is not None:
        return fst + snd
    addition = fst or snd
    if addition is None:
        raise ValueError
    return addition


def parse_results() -> dict[str, dict[float, list[dict[str, float]]]]:
    experiments_dict: dict[str, dict[float, list[dict[str, float]]]] = {}

    for experiment in EXPERIMENTS:
        curr_experiment_dict: dict[float, list[list[InfectionStatus]]] = {}

        for file in os.listdir(DIR + experiment):
            if not file.endswith(".txt"):
                continue

            filename, _ = os.path.splitext(file)

            # NOTE: Filenames look like "tra0.30_run0"
            variable_value = filename.split("_")[0][3:]

            try:
                variable_value = float(variable_value)
            except:
                raise ValueError("Parsing error")

            if not variable_value in curr_experiment_dict.keys():
                curr_experiment_dict[variable_value] = []

            current_run_arr = []
            with open(DIR + f"{experiment}/{file}", "r") as f:
                for line in f:
                    values_list = list(map(lambda s: int(s), line.split(" ")))
                    current_run_arr.append(
                        InfectionStatus(
                            values_list[0],
                            values_list[1],
                            values_list[2],
                            values_list[3],
                            values_list[4],
                        )
                    )

            curr_experiment_dict[variable_value].append(current_run_arr)

        # Add current experiment to dict
        experiments_dict[experiment] = {
            key: aggregate_runs(all_runs_list)
            for key, all_runs_list in curr_experiment_dict.items()
        }

    return experiments_dict


def aggregate_runs(runs_list: list[list[InfectionStatus]]) -> list[dict[str, float]]:
    """Each run is a list of InfectionStatus (one per day of said run). I need a
    list where each element contains each average and each standart deviation for
    each variable in InfectionStatus"""

    # Extend the arrays of different runs to have the same lenght.
    max_lenght = max(map(lambda l: len(l), runs_list))
    for idx in range(0, len(runs_list)):
        run_len = len(runs_list[idx])
        if run_len < max_lenght and run_len != 0:
            len_diff = max_lenght - run_len
            runs_list[idx][run_len:] = [
                runs_list[idx][run_len - 1] for _ in range(0, len_diff)
            ]

    aggregates_list = []
    for day_idx in range(0, max_lenght):
        susceptibles: list[float] = []
        infected: list[float] = []
        recovered: list[float] = []
        dead: list[float] = []

        for run in runs_list:
            day_in_run = run[day_idx]
            susceptibles.append(day_in_run.susceptible)
            infected.append(day_in_run.infected)
            recovered.append(day_in_run.recovered)
            dead.append(day_in_run.dead)

        aggregates_list.append(
            {
                "day": runs_list[0][day_idx].day,
                "susceptibles": np.mean(susceptibles),
                "infected": np.mean(infected),
                "recovered": np.mean(recovered),
                "dead": np.mean(dead),
                "susceptibles_std": np.std(susceptibles),
                "infected_std": np.std(infected),
                "recovered_std": np.std(recovered),
                "dead_std": np.std(dead),
            }
        )
    return aggregates_list


RESULTS_PATH = "./analysis/figs/"


def plot(data: dict[str, dict[float, list[dict[str, float]]]]):
    os.makedirs(RESULTS_PATH, exist_ok=True)

    # cumulative_graphs(data["period"][2])
    graph_total_vs_variable(data["mortality"], "Tasa de Mortalidad", 0.02)
    graph_total_vs_variable(data["transmission"], "Tasa de Infección", 0.05)
    graph_total_vs_variable(data["period"], "Período de Contagio (días)", 1)


def cumulative_graphs(data: list[dict[str, float]]):
    fig1 = plt.figure(figsize=(1920 / 108, 1080 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})

    days = []
    susceptibles = []
    infected = []
    recovered = []
    dead = []

    for inf_status in data:
        days.append(inf_status["day"])
        susceptibles.append(inf_status["susceptible"])
        infected.append(inf_status["infected"])
        recovered.append(inf_status["recovered"])
        dead.append(inf_status["dead"])

    plt.plot(susceptibles, "o-", label="Susceptibles", color="blue")
    plt.plot(infected, "o-", label="Infectados", color="red")
    plt.plot(recovered, "o-", label="Recuperados", color="green")
    plt.plot(dead, "o-", label="Fallecidos", color="black")

    # Customize the plot
    plt.xlabel("Tiempo (días)")
    plt.ylabel("Cantidad de individuos")
    plt.legend()

    # Show the plot
    fig1.savefig(RESULTS_PATH + f"temporal.png")

    fig, ax = plt.subplots(figsize=(1920 / 108, 1080 / 108), dpi=108)
    plt.xlabel("Tiempo (días)")
    plt.ylabel("Cantidad de individuos")
    ax.stackplot(
        days,
        infected,
        dead,
        recovered,
        susceptibles,
        labels=["Infectados", "Fallecidos", "Recuperados", "Susceptibles"],
        colors=["red", "grey", "green", "blue"],
    )
    plt.legend(loc="center left")
    plt.xlim(0, days[-1])
    plt.ylim(0, 1000)
    fig.show()
    fig.savefig(RESULTS_PATH + f"cumulative_graph.png")


def graph_total_vs_variable(
    data: dict[float, list[dict[str, float]]], xlabel: str, xstep: float
):
    fig1 = plt.figure(figsize=(1920 / 108, 1080 / 108), dpi=108)
    plt.rcParams["font.family"] = "serif"
    plt.rcParams.update({"font.size": 16})

    variable_input_list = []
    susceptible_medians = []
    susceptible_stds = []
    # recovered_medians = []
    # recovered_stds = []
    dead_medians = []
    dead_stds = []

    for rate, outputs in data.items():
        variable_input_list.append(rate)

        susceptible_medians.append(outputs[-1]["susceptibles"])
        susceptible_stds.append(outputs[-1]["susceptibles_std"])

        # recovered_medians.append(outputs[-1]["recovered"])
        # recovered_stds.append(outputs[-1]["recovered_std"])

        dead_medians.append(outputs[-1]["dead"])
        dead_stds.append(outputs[-1]["dead_std"])

    plt.errorbar(
        variable_input_list,
        susceptible_medians,
        susceptible_stds,
        fmt="x",
        label="Susceptibles",
        color="blue",
        markersize=8,
        capsize=5,
    )

    # plt.errorbar(
    #     variable_input_list,
    #     recovered_medians,
    #     recovered_stds,
    #     fmt="x",
    #     label="Recuperados",
    #     color="green",
    #     markersize=8,
    #     capsize=5,
    # )

    plt.errorbar(
        variable_input_list,
        dead_medians,
        dead_stds,
        fmt="x",
        label="Fallecidos",
        color="black",
        markersize=8,
        capsize=5,
    )

    plt.xlabel(xlabel)
    plt.ylabel("Cantidad de individuos")
    plt.gca().xaxis.set_major_locator(MultipleLocator(xstep))
    plt.grid()
    plt.legend()

    # Show the plot
    fig1.savefig(RESULTS_PATH + f"{xlabel}.png")


if __name__ == "__main__":
    rs = parse_results()

    plot(rs)
