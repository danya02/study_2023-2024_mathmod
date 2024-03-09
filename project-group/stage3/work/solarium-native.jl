using Random

mutable struct Particle
    position
    velocity
    mass::Float64
    radius::Float64
end

function glue(a::Particle, b::Particle)::Particle
    Particle(
        (a.position*a.mass + b.position*b.mass)/(a.mass+b.mass),
        (a.velocity*a.mass + b.velocity*b.mass)/(a.mass+b.mass),
        a.mass+b.mass,
        cbrt(a.radius^3 + b.radius^3)
    )
end

max_mass = 1
max_radius = 1
function get_random_particle(radius_max, angular_speed)::Particle
    radius = sqrt(Random.rand()) * radius_max
    angle = Random.rand() * 2 * pi
    x = radius * cos(angle)
    y = radius * sin(angle)
    vx = -y * angular_speed * (radius_max / radius)^(3/2)
    vy = x * angular_speed * (radius_max / radius)^(3/2)
    Particle([x,y], [vx,vy], Random.rand()*max_mass, Random.rand()*max_radius)
end
get_random_particle(1, 1)

function magnitude(vec)
    sqrt(vec[1]^2 + vec[2]^2)
end
function normalize(vec)
    vec / magnitude(vec)
end


universal_gravitation_const = 6.674 * 10^(-11)
function apply_particle_forces(particles, dt)
    for src_part in particles
        force = [0.0, 0.0]
        for dst_part in particles
            if src_part === dst_part
                continue
            end
            extra_force = normalize(src_part.position - dst_part.position) * universal_gravitation_const * (src_part.mass * dst_part.mass) / magnitude(src_part.position - dst_part.position)
            force += extra_force
        end
        src_part.velocity += force * dt
    end
end

function step_particles(particles, dt)
    for p in particles
        p.position += p.velocity * dt
    end
end

particles = [get_random_particle(10, 10) for _ in 1:1000]
dt = 0.01
for i in 1:100
    apply_particle_forces(particles, dt)
    step_particles(particles, dt)
    println(particles[1])
end




