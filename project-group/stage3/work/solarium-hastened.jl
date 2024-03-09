#run(`sh -c "ls && cd project-group/stage3/work/solarium && cargo build --release"`)
run(`sh -c "cd solarium && cargo build --release"`)

using Libdl
using LinearAlgebra
import JSON
import Random
import FourierTools

struct Vec2
    x::Float64
    y::Float64
end

struct Particle
    id::UInt64
    position::Vec2
    velocity::Vec2
    mass::Float64
    radius::Float64
end

#const LIB = Libdl.dlopen("project-group/stage3/work/solarium/target/release/libsolarium.so")
const LIB = Libdl.dlopen("solarium/target/release/libsolarium.so")

function random_particles(count)
    len = count
    radius_max = 10
    angular_speed = 0
    sym = Libdl.dlsym(LIB, :make_random_particle_list)
    arr = @ccall $sym(radius_max::Float64, angular_speed::Float64, len::Csize_t)::Ptr{Particle}
    unsafe_wrap(Array{Particle}, arr, len, own=true)
end

max_part_mass = 1e1
min_density = 0.1  # 100g/m^3
max_density = 0.5  # 500g/m^3
function get_random_particle(radius_max, angular_speed, i)::Particle
    radius = sqrt(Random.rand()) * radius_max
    angle = Random.rand() * 2 * pi
    x = radius * cos(angle)
    y = radius * sin(angle)
    vx = -y * angular_speed * (radius_max / radius)^(3/2)
    vy = x * angular_speed * (radius_max / radius)^(3/2)
    # vx = 0
    # vy = 0
    mass = Random.rand() * max_part_mass
    density = Random.rand() * (max_density - min_density) + min_density
    volume = mass / density
    radius = cbrt((3/4pi) * volume)
    # if i==0
    #     mass = 5
    #     vx = 0
    #     vy = 0
    #     x = 0
    #     y = 0
    # end
    Particle(i, Vec2(x,y), Vec2(vx,vy), mass, radius)
end
function get_uninit_particle(i)::Particle
    Particle(i, Vec2(420,420), Vec2(420,420), 420, 420)
end


function perform_timesteps(particles::Vector{Particle}, steps)
    # allocate "steps" copies of the particles Array
    particle_count = Csize_t(length(particles))
    step_count = Csize_t(steps)
    step_array = Vector{Vector{Particle}}(undef, steps);
    
    # important to allocate each of the arrays here
    for i in 1:steps
        step_array[i] = Vector{Particle}(undef, length(particles));
    end
    step_array[1] = particles;
    for i in 2:steps
        for j in 1:length(particles)
            step_array[i][j] = get_uninit_particle(j)
        end
    end

    sym = Libdl.dlsym(LIB, :perform_timesteps)
    @ccall $sym(step_array::Ptr{Ptr{Particle}}, particle_count::Csize_t, step_count::Csize_t)::Cvoid

    # # save simulation results
    # f = open(io -> write(io, JSON.json(step_array)), "output.json", write=true)
    step_array
end

println("Spawning particles")
particles = [get_random_particle(1000, 2, i) for i=0:5000]

println("Running simulation")
frames = @time perform_timesteps(particles, 50)

# const TimeSeriesPoint = Tuple{UInt64, Float64}

struct TimeSeriesPoint
    t::Csize_t
    value::Float64
end

println("Collecting fcurves")
global raw_fcurves_x = Dict{UInt64, Array{TimeSeriesPoint}}()
global raw_fcurves_y = Dict{UInt64, Array{TimeSeriesPoint}}()
global raw_fcurves_vx = Dict{UInt64, Array{TimeSeriesPoint}}()
global raw_fcurves_vy = Dict{UInt64, Array{TimeSeriesPoint}}()
global raw_fcurves_mass = Dict{UInt64, Array{TimeSeriesPoint}}()
global raw_fcurves_radius = Dict{UInt64, Array{TimeSeriesPoint}}()
function collect_fcurves()
    for particle in frames[1]
        raw_fcurves_x[particle.id] = Array{TimeSeriesPoint}(undef, length(frames))
        raw_fcurves_y[particle.id] = Array{TimeSeriesPoint}(undef, length(frames))
        raw_fcurves_vx[particle.id] = Array{TimeSeriesPoint}(undef, length(frames))
        raw_fcurves_vy[particle.id] = Array{TimeSeriesPoint}(undef, length(frames))
        raw_fcurves_mass[particle.id] = Array{TimeSeriesPoint}(undef, length(frames))
        raw_fcurves_radius[particle.id] = Array{TimeSeriesPoint}(undef, length(frames))
    end

    for frame_idx in eachindex(frames)
        # println("Collecting fcurves: ", frame_idx)
        for particle_idx in eachindex(frames[1])
            frame = frames[frame_idx]
            particle = frame[particle_idx]
            push!(raw_fcurves_x[particle.id], TimeSeriesPoint(frame_idx,particle.position.x))

            push!(raw_fcurves_y[particle.id], TimeSeriesPoint(frame_idx,particle.position.y))

            push!(raw_fcurves_vx[particle.id], TimeSeriesPoint(frame_idx,particle.velocity.x))

            push!(raw_fcurves_vy[particle.id], TimeSeriesPoint(frame_idx,particle.velocity.y))

            push!(raw_fcurves_mass[particle.id], TimeSeriesPoint(frame_idx,particle.mass))

            push!(raw_fcurves_radius[particle.id], TimeSeriesPoint(frame_idx, particle.radius))

        end
    end
    
end

@time collect_fcurves()


# # https://rosettacode.org/wiki/Ramer-Douglas-Peucker_line_simplification#Julia


# function perpdist(pt::TimeSeriesPoint, lnstart::TimeSeriesPoint, lnend::TimeSeriesPoint)
#     lnstart1::Array{Float64} = [lnstart[1], lnstart[2]]
#     lnend1::Array{Float64} = [lnend[1], lnend[2]]
#     pt1::Array{Float64} = [pt[1], pt[2]]
#     d = normalize!(lnend1 .- lnstart1)

#     pv = pt1 .- lnstart1
#     # Get dot product (project pv onto normalized direction)
#     pvdot = dot(d, pv)
#     # Scale line direction vector
#     ds = pvdot .* d
#     # Subtract this from pv
#     return norm(pv .- ds)
# end

# function rdp(plist::Vector{TimeSeriesPoint}, ϵ::Float64, depth::Vector{Int64})
#     depth[1] -= 1
#     if depth[1] < 0
#         return [plist[1], plist[end]]
#     end
#     if length(plist) < 2
#         throw(ArgumentError("not enough TimeSeriesPoints to simplify"))
#     end

#     # Find the TimeSeriesPoint with the maximum distance from line between start and end
#     dmax = 0
#     imax = -1
#     for idx in eachindex(plist)
#         pt = plist[idx]
#         dist = perpdist(pt, plist[1], plist[end])
#         if dist > dmax
#             dmax = dist
#             imax = idx
#         end
#     end

#     # If max distance is greater than epsilon, recursively simplify
#     if dmax > ϵ
#         fstline = plist[1:imax]
#         lstline = plist[imax:end]

#         recrst1 = rdp(fstline, ϵ, depth)
#         recrst2 = rdp(lstline, ϵ, depth)

#         out = vcat(recrst1, recrst2)
#     else
#         out = [plist[1], plist[end]]
#     end

#     depth[1] += 1
#     return out
# end

struct DecimationData
    number_of_series::Csize_t
    lengths_of_series::Ptr{Csize_t}
    source_series::Ptr{Pair{UInt64, Ptr{TimeSeriesPoint}}}
    dst_series::Ptr{Pair{UInt64, Ptr{TimeSeriesPoint}}}
end

function decimate_timeseries(fcurves::Dict{UInt64, Array{TimeSeriesPoint}}, eps)
    for key in eachindex(fcurves)
        out_size = [-1]
        fcurve = Array{TimeSeriesPoint}(undef, length(fcurves[key]))
        for k in eachindex(fcurves[key])
            fcurve[k] = fcurves[key][k]
        end
        src::Ptr{TimeSeriesPoint} = pointer(fcurve)
        sym = Libdl.dlsym(LIB, :decimate_timeseries)
        arr = @ccall $sym(src::Ptr{TimeSeriesPoint}, length(fcurves[key])::Csize_t, eps::Float64, pointer(out_size)::Ptr{Csize_t})::Ptr{TimeSeriesPoint}

        new_tsp = unsafe_wrap(Array{TimeSeriesPoint}, arr, out_size[1], own=true)
        fcurves[key] = new_tsp
    end
end

eps = 0.01
maxdepth = 100

@time begin
    println("Running TimeSeriesPoint decimation for X")
    # for particle in raw_fcurves_x
    #     particle = [particle[1], rdp(particle[2]::Vector{TimeSeriesPoint}, eps, [maxdepth])]
    # end
    decimate_timeseries(raw_fcurves_x, eps)
end


@time begin
    # println("Running TimeSeriesPoint decimation for Y")
    # for particle in raw_fcurves_y
    #     particle = [particle[1], rdp(particle[2]::Vector{TimeSeriesPoint}, eps, [maxdepth])]
    # end
    decimate_timeseries(raw_fcurves_y, eps)

end

@time begin
    # println("Running TimeSeriesPoint decimation for vX")
    # for particle in raw_fcurves_vx
    #     particle = [particle[1], rdp(particle[2]::Vector{TimeSeriesPoint}, eps, [maxdepth])]
    # end
    decimate_timeseries(raw_fcurves_vx, eps)

end

@time begin
    # println("Running TimeSeriesPoint decimation for vY")
    # for particle in raw_fcurves_vy
    #     particle = [particle[1], rdp(particle[2]::Vector{TimeSeriesPoint}, eps, [maxdepth])]
    # end
    decimate_timeseries(raw_fcurves_vy, eps)

end

@time begin
    # println("Running TimeSeriesPoint decimation for mass")
    # for particle in raw_fcurves_mass
    #     particle = [particle[1], rdp(particle[2]::Vector{TimeSeriesPoint}, eps, [maxdepth])]
    # end
    decimate_timeseries(raw_fcurves_mass, eps)

end

@time begin
    # println("Running TimeSeriesPoint decimation for radius")
    # for particle in raw_fcurves_radius
    #     particle = [particle[1], rdp(particle[2]::Vector{TimeSeriesPoint}, eps, [maxdepth])]
    # end
    decimate_timeseries(raw_fcurves_radius, eps)

end

data = Dict(
    "x" => raw_fcurves_x,
    "y" => raw_fcurves_y,
    "vx" => raw_fcurves_vx,
    "vy" => raw_fcurves_vy,
    "mass" => raw_fcurves_mass,
    "radius" => raw_fcurves_radius,
)

println("Saving results")
@time open(io -> write(io, JSON.json(data)), "output.json", write=true)


# @ccall "".main()::Cvoid

