import itertools
import os

DIR = "./analysis/data/"

EXPERIMENTS = ["transmission", "period", "mortality"]
RUN_COUNT = 3


def parse_results():
    experiments_dict = {}

    for experiment in EXPERIMENTS:
        current_experiment_dict: dict[float, list[InfectionStatus]] = {}

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

            if not variable_value in current_experiment_dict.keys():
                current_experiment_dict[variable_value] = []

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

            # Add different runs together
            previous_run_arr = current_experiment_dict[variable_value]

            len_prev = len(previous_run_arr)
            len_curr = len(current_run_arr)
            max_lenght = max(len_prev, len_curr)

            if len_prev < max_lenght and len_prev != 0:
                len_diff = max_lenght - len_prev
                previous_run_arr[len_prev:] = [
                    previous_run_arr[len_prev - 1] for _ in range(0, len_diff)
                ]
            elif len_curr < max_lenght and len_curr != 0:
                len_diff = max_lenght - len_curr
                current_run_arr[len_curr:] = [
                    current_run_arr[len_curr - 1] for _ in range(0, len_diff)
                ]

            current_experiment_dict[variable_value] = [
                add_status(x, y)
                for (x, y) in itertools.zip_longest(current_run_arr, previous_run_arr)
            ]

        # Divide all InfectionStatus by number of runs to calculate average.
        for run_values in current_experiment_dict.values():
            for value in run_values:
                value.divide_by(RUN_COUNT)

        # Add current experiment to dict
        experiments_dict[experiment] = current_experiment_dict
    return experiments_dict


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


if __name__ == "__main__":
    rs = parse_results()

    for exp, values_dict in rs.items():
        print(exp)
        for config, values in values_dict.items():
            print(f"Config {config}")
            print(f"Values:")
            for value in values:
                print(value)
        print()
