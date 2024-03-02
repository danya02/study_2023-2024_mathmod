model lab4_1
Real x(start=1.7);
Real dx(start=-0.2);
equation

der(dx) + 1.7*x = 0;
der(x) = dx;

end lab4_1;