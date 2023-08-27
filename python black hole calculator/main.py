import customtkinter

from utils import *

# Extra functions
def BOWLING_BALLING():
  update(21.6)
  
  slider_value.configure(text="Radius: BOWLING BALL")
  title.configure(text="Use the slider to insert a BOWLING BALL or use the input field below the BOWLING BALL.")
  app.title("THE BOWLING BALL")
  text_input.configure(placeholder_text="a BOWLING BALL is 21.6 centimeters")

  for i in range(10): print("THE BOWLING BALL"); i +=1

def change_slider_length() -> None:
  try:
    new_max = float(slider_range_input.get())
  except:
    print(f"Conversion failure. Input from user not converable to Float. Likely not a number (boolean value or string)")
    return None

  if new_max < 5:
    new_max = 5
    output.configure(text="Slider set to minimum value of 5")
  else:
    output.configure(text=f"Slider set to of {new_max}")
  slider.configure(to=new_max, number_of_steps=new_max * 2)

def update(value = "Fallback") -> None:
  if app.title() != "Black Hole Mass Calculator": 
    app.title("Black Hole Mass Calculator")
    title.configure(text="Use the slider to insert a radius or use the input field below the slider.")
    text_input.configure(placeholder_text="Radius in centimeters")

  slider_value.configure(text=f"Radius: {slider.get()}")

  if value == "Fallback":
    try:
      value = float(text_input.get())
      slider_value.configure(text=f"Radius: {value}")
    except:
      print(f"Conversion failure. Input from user not converable to Float. Likely not a number (boolean value or string)")
      return None

  calculated_mass: str = get_mass(value)
  mass_to_sun: str = compare_mass_to_sun(get_mass(value, False))

  output.configure(text=f'The mass of a black hole with a radius of {value}cm is {calculated_mass}kg') if value != 21.6 else output.configure(text=f'The mass of a black hole with a radius of A BOWLING BALL is {calculated_mass}kg')
  fun_fact.configure(text=f'That is {mass_to_sun}% the mass of the Sun!')

# System settings
customtkinter.set_appearance_mode("Sysem")
customtkinter.set_default_color_theme("blue")

# App frame settings
app = customtkinter.CTk()
app.geometry("720x480")
app.title("Black Hole Mass Calculator")

# Ui elements
title = customtkinter.CTkLabel(app, text="Use the slider to insert a radius or use the input field below the slider.")
title.pack(padx=10, pady=10)

slider_value = customtkinter.CTkLabel(app, text="Radius: 0.0cm")
slider_value.pack()

slider = customtkinter.CTkSlider(app, from_=0, to=5000, orientation="horizontal", width=400, command=update, number_of_steps=10000)
slider.pack()

text_input = customtkinter.CTkEntry(app, placeholder_text="Radius in centimeters", width=275)
text_input.pack(pady=10)

calc_button = customtkinter.CTkButton(app, command=update, text="Calculate").pack()

output = customtkinter.CTkLabel(app, text="Output will be rendered here.", wraplength=400)
output.pack(pady=30)

fun_fact = customtkinter.CTkLabel(app, text="", wraplength=400, font=('default', 10, 'italic'))
fun_fact.pack(pady=10)

slider_range_input = customtkinter.CTkEntry(app, placeholder_text="Change maximum slider value", font=('default', 10), width=160)
slider_range_input.pack(pady=10)

submit_slider_range = customtkinter.CTkButton(app, text="Change", command=change_slider_length, width=50).pack()
BOWLING_BALL = customtkinter.CTkButton(app, text="BOWLING BALL", command=BOWLING_BALLING, fg_color=('firebrick1'), hover_color=('firebrick')).pack(pady=20)

# Run app
app.mainloop()
