import sys
import json
import math

import tqdm

data = json.load(open(sys.argv[1]))['timestep_states']

def kin_energy(particle):
    return 0.5 * particle['mass'] * math.hypot(particle['velocity']['x'], particle['velocity']['y'])

G = 6.674e-11 * 2e10

def pot_energy(p1, p2):
    return G * p1['mass'] * p2['mass'] / math.hypot(p1['position']['x'] - p2['position']['x'], p1['position']['y'] - p2['position']['y'])

try:
    total_potential = []
    total_kinetic = []
    for state in tqdm.tqdm(data):
        kinetic = 0
        potential = 0
        for particle in state:
            kinetic += kin_energy(particle)
        for first_particle in state:
            if first_particle['radius'] == 0: continue
            for second_particle in state:
                if second_particle['radius'] == 0: continue
                if first_particle['id'] != second_particle['id']:
                    potential += pot_energy(first_particle, second_particle)
        total_kinetic.append(kinetic)
        total_potential.append(potential)
finally:
    print('potential:', total_potential)
    print('kinetic:', total_kinetic)

