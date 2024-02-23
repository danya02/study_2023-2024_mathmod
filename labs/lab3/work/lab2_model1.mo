model lab2model1
  Real x(start=25000);
  Real y(start=39000);
equation
  der(x) = -0.441*x - 0.773*y + sin(2*time) + 1;
  der(y) = -0.55*x  - 0.664*y + cos(2*time) + 1;
  if x<=0 then
    terminate("X was defeated");
  end if;
  if y<=0 then
    terminate("Y was defeated");
  end if;
  
end lab2model1;