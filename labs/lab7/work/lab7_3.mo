model lab7_3
  Real n(start=12);
  constant Real N=1420;
equation
    der(n) = (0.57 * sin(time) + 0.38 * cos(13 * time) * n) * (N - n);
end lab7_3;