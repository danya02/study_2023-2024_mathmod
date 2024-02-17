model Lab1
  Real cutter_r1(start=17.6);
  Real cutter_phi1(start=0);
  Real cutter_r2(start=17.6);
  Real cutter_phi2(start=-2 * asin(1));

  Boolean did_match_dist1(start=false);
  Boolean did_match_dist2(start=false);

  Real boat_r(start=0);
  Real distance1;
  Real distance2;
  
  Real pi = 2*asin(1);
  
  parameter Real boat_phi = 1.23; 
equation
  der(boat_r) = 1;
  distance1 = sqrt(boat_r * boat_r + cutter_r1 * cutter_r1 - 2*boat_r*cutter_r1 * cos(boat_phi - cutter_phi1));
  distance2 = sqrt(boat_r * boat_r + cutter_r2 * cutter_r2 - 2*boat_r*cutter_r2 * cos(boat_phi - cutter_phi2));

  if did_match_dist1 then
    der(cutter_r1) = 1;
    cutter_r1 * der(cutter_phi1) = sqrt(41/60);
   else 
    der(cutter_r1) = -5.1;
    cutter_phi1 = 0;
   end if;
   
   when cutter_r1 < boat_r then
    did_match_dist1 = true;
   end when;

  if did_match_dist2 then
    der(cutter_r2) = 1;
    cutter_r2 * der(cutter_phi2) = -sqrt(41/60);
   else 
    der(cutter_r2) = -5.1;
    cutter_phi2 = -pi;
   end if;
   
   when cutter_r2 < boat_r then
    did_match_dist2 = true;
   end when;
end Lab1;