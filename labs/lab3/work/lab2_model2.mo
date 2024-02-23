model lab2model2
  Real x(start=25000);
  Real y(start=39000);
equation
  der(x) = -0.399*x - 0.688*y + sin(2*time) + 2;
  der(y) = -0.299 * x * y - 0.811*y + cos(3*time) + 1;
  
  if x<=0 then
    terminate("X was defeated");
  end if;
  if y<=0 then
    terminate("Y was defeated");
  end if;
end lab2model2;