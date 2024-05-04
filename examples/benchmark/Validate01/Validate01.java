public class Validate01 {
  public static void test(String firstName, String lastName, String address, String city, String state, String zip, String phone) {
    if (!ValidateInput01.validateFirstName(firstName)) assert false;
    else if (!ValidateInput01.validateLastName(lastName)) assert false;
    else if (!ValidateInput01.validateAddress(address)) System.out.println("Invalid address");
    else if (!ValidateInput01.validateCity(city)) assert false;
    else if (!ValidateInput01.validateState(state)) assert false;
    else if (!ValidateInput01.validateZip(zip)) System.out.println("Invalid zip code");
    else if (!ValidateInput01.validatePhone(phone)) System.out.println("Invalid phone number");
    else System.out.println("Valid input.  Thank you.");
  }
}
