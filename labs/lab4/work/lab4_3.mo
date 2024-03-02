model lab4_3
Real x(start=1.7);
Real dx(start=-0.2);
equation

der(dx) + 1.7*dx + 1.7*x = 0.7 * cos(2.7 * time);
der(x) = dx;

end lab4_3;