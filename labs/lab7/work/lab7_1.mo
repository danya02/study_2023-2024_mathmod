model lab7_1
  Real n(start=12);
  constant Real N=1420;
equation
    der(n) = (0.7 + 0.000012 * n)*(N - n);
end lab7_1;