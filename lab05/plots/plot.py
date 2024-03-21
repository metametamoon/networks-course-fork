import matplotlib.pyplot as plt
import numpy as np


def calculate_time_p2p(F, u_s, d_min, u_client, N):
    s = max([F / u_s, F / d_min, N * F / (u_s + N * u_client)])
    print(s)
    return s


def calculate_time_client_server(F, u_s, d_min, N):
    result = max(N * F / u_s, F / d_min)
    print(result)
    return result


F = 15_000_000_000
u_s = 30_000_000
d_min = 2_000_000

for N in 10, 100, 1000:
    species = ("u = 300 Кбит/c", "u = 700 Кбит/c", "u = 2 Мбит/c")
    u = [300_000, 700_000, 2000_000]
    penguin_means = {
        'server-client': [calculate_time_client_server(F, u_s, d_min, N) for u_client in u],
        'peer-to-peer': [calculate_time_p2p(F, u_s, d_min, u_client, N) for u_client in u],
    }

    x = np.arange(len(species))  # the label locations
    width = 0.25  # the width of the bars
    multiplier = 0

    fig, ax = plt.subplots(layout='constrained')

    for attribute, measurement in penguin_means.items():
        offset = width * multiplier
        rects = ax.bar(x + offset, measurement, width, label=attribute)
        ax.bar_label(rects, padding=3)
        multiplier += 1

    # Add some text for labels, title and custom x-axis tick labels, etc.
    ax.set_ylabel('Time (s)')
    ax.set_title(f'Time comparing for N={N}')
    ax.set_xticks(x + width, species)
    ax.legend(loc='upper left', ncols=3)
    # ax.set_ylim(0, 500000)
    plt.savefig(f'N_{N}')
    plt.show()
