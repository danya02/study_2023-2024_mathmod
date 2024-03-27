using Plots
using JSON

data = JSON.parsefile("./solarium/demo5-more/7-fullenergies.json")
pot = data["potential"]
kin = data["kinetic"]

offset = kin[2]
kin = map((v) -> v-offset, kin)
kin[1] = 0
kin[2] = 0


f = plot([pot, kin, pot+kin])
savefig(f, "out1.png")
f = plot([data["active_particles"]])
savefig(f, "out2.png")
