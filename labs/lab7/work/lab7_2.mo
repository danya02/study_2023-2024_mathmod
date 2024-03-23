model lab7_2
  Real n(start=12);
  constant Real N=1420;
equation
    der(n) = (0.0003 + 0.5 * n)*(N - n);
end lab7_2;