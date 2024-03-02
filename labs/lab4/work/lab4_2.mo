model lab4_2
Real x(start=1.7);
Real dx(start=-0.2);
equation

der(dx) + 1.7*dx + 1.7*x = 0;
der(x) = dx;

end lab4_2;