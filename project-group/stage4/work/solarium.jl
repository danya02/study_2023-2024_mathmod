#run(`sh -c "ls && cd project-group/stage4/work/solarium && cargo build --release"`)
run(`sh -c "cd solarium && cargo build --release"`)
run(`sh -c "cd solarium && cargo build"`)

import Libdl
import JSON

# Global simulation parameters
INIT_PARTICLE_COUNT = 10000#00
INIT_RADIUS = 1000.0
INIT_ANGULAR_VELOCITY = 0.2
TIMESTEP_COUNT = 100
SINGLE_TIMESTEP_DURATION = 0.01;

const InteropData = Ref{UInt8}

#const LIB = Libdl.dlopen("project-group/stage4/work/solarium/target/release/libsolarium.so")
const LIB = Libdl.dlopen("solarium/target/release/libsolarium.so")

function make_interop()::InteropData
    sym = Libdl.dlsym(LIB, :get_interop_size)
    size = @ccall $sym()::Csize_t
    memory = Vector{UInt8}(undef, size)
    memory
end

function voidcall(name::String, what::InteropData)
    sym = Libdl.dlsym(LIB, name)
    @ccall $sym(what::InteropData)::Cvoid
end


function u64call(name::String, what::InteropData, value)
    sym = Libdl.dlsym(LIB, name)
    @ccall $sym(what::InteropData, value::UInt64)::UInt64
end


data = @time make_interop()
@time voidcall("prepare_interop_buf", data)

function init_particles(data::InteropData, count, radius_max::Float64, angular_speed::Float64)
    sym = Libdl.dlsym(LIB, :initialize_particles)
    @ccall $sym(data::InteropData, count::UInt64, radius_max::Float64, angular_speed::Float64)::Cvoid
end

function perform_timesteps(data::InteropData, count, dt::Float64)
    sym = Libdl.dlsym(LIB, :perform_timesteps)
    @ccall $sym(data::InteropData, count::UInt64, dt::Float64)::UInt64
end

function load(data::InteropData, filename)
    sym = Libdl.dlsym(LIB, :load_from_file)
    @ccall $sym(data::InteropData, filename::Cstring)::Cvoid
end

function save(data::InteropData, filename)
    sym = Libdl.dlsym(LIB, :save_to_file)
    @ccall $sym(data::InteropData, filename::Cstring)::Cvoid
end

function save_as_json(data::InteropData, filename)
    sym = Libdl.dlsym(LIB, :save_as_json)
    @ccall $sym(data::InteropData, filename::Cstring)::Cvoid
end

function save_ts(data::InteropData, filename)
    sym = Libdl.dlsym(LIB, :save_data_to_timeseries)
    @ccall $sym(data::InteropData, filename::Cstring)::Cvoid
end



print("Generating random particles: ")
@time init_particles(data, INIT_PARTICLE_COUNT, INIT_RADIUS, INIT_ANGULAR_VELOCITY)

# print("Loading from disk:")
# @time load(data, "test1/1.postcard")

@time perform_timesteps(data, TIMESTEP_COUNT, SINGLE_TIMESTEP_DURATION)

print("Saving to disk")
@time save(data, "1.postcard")

print("Saving timeseries to disk")
@time save_ts(data, "1-series.json")

# @time voidcall("debug_interop_buf", data)
