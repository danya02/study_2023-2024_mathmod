model lab8_2
  Real m1(start=8);
  Real m2(start=10);
  constant Real pcr=50;
  constant Real N=50;
  constant Real q=1;
  constant Real t1=36;
  constant Real t2=30;
  constant Real p1=10;
  constant Real p2=12;
  Real a1;
  Real a2;
  Real b;
  Real c1;
  Real c2;
equation
    a1 = pcr / (t1*t1 * p1*p1 * N * q);
    a2 = pcr / (t2*t2 * p2*p2 * N * q);
    b = pcr / (t1*t1 * t2*t2 * p1*p1 * p2*p2 * N * q);
    c1 = (pcr-p1) / (t1*p1);
    c2 = (pcr-p2) / (t2*p2);

    der(m1) = m1 - ((b/c1) + 0.00041) * m1 * m2 - (a1/c1)*m1*m1;
    der(m2) = (c2/c1)*m2 - (b/c1)*m1*m2 - (a2/c1)*m2*m2;
end lab8_2;