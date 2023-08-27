def get_mass(radius: float, commas = True) -> int | str:
  """Calculate the mass of a black hole with a radius of x
  
  ### Parameters
  radius: int
    The radius of the black hole in centimeters
  commas: bool
    boolean to decide if you want to format the result with commas

  ### Returns
  if commas = True
    A formatted string of the mass of the black hole
  if commas = False
    An int equal to the mass of the black hole
  """

  result = int(299792458.0 ** 2 / (2 * 6.6743e-11) * radius)
  return f"{result:,}" if commas else result

def compare_mass_to_sun(value: int) -> str:
  """Compares a given mass to the mass of the sun
  
  ### Parameters:
  value: int
    The mass you want to compare to that of the sun

  ### Returns
  Formatted string of a float representing the % mass compared to that of the sun.
  """

  return f"{(value / 1.988409870698051e+30) * 100:.4f}"
