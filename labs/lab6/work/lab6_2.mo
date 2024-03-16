model lab6_2
  Real i(start=124);
  Real r(start=30);
  Real s(start=7970);  // 8124-124-30
  constant Real alpha=0.1;
  constant Real beta=0.2;
  constant Real istar=100;
equation
  if i>istar then
    der(s) = -alpha * s;
    der(i) = alpha*s - beta * i;
  else
    der(s) = 0;
    der(i) = -beta * i;
  end if;
  der(r) = beta * i;

end lab6_2;