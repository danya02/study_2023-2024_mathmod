import pygame
import json
from numpy import interp

pygame.init()

data = json.load(open('output.json'))

WIDTH = HEIGHT = 400
display = pygame.display.set_mode((WIDTH*2, HEIGHT*2))
scale = 1

clock = pygame.time.Clock()

font = pygame.font.SysFont(pygame.font.get_default_font(), 64)

minx = float('inf')
miny = float('inf')
maxx = float('-inf')
maxy = float('-inf')

while 1:
    for i, frame in enumerate(data):
        # center of mass
        center_x = center_y = 0
        for particle in frame:
            center_x += particle['position']['x']
            center_y += particle['position']['y']
            minx = min(minx, particle['position']['x'])
            maxx = max(maxx, particle['position']['x'])
            miny = min(miny, particle['position']['y'])
            maxy = max(maxy, particle['position']['y'])
        center_y /= len(frame)
        center_x /= len(frame)
        for particle in frame:
            x,y = particle['position']['x'], particle['position']['y']
            sx = int(interp(x, [minx, maxx], [0, WIDTH*2]))
            sy = int(interp(y, [miny, maxy], [0, HEIGHT*2]))

            vx = x + (particle['velocity']['x'])
            vy = y + (particle['velocity']['y'])
            svx = int(interp(vx, [minx, maxx], [0, WIDTH*2]))
            svy = int(interp(vy, [miny, maxy], [0, HEIGHT*2]))

            pygame.draw.circle(display, 'white', (x,y), 5)
            pygame.draw.line(display, 'green', (sx,sy), (svx,svy))


        display.blit(font.render(str(i), 1, 'red'), (0,0))
        pygame.display.flip()
        display.fill('black')
        clock.tick(10)