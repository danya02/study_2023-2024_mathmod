model lab5
  Real x(start=10);
  Real y(start=10);
equation
  der(x) = -0.28*x + 0.028 * x * y;
  der(y) = 0.29*y - 0.029 * x * y;
end lab5;