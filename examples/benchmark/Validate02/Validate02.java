public class Validate02 {
  public static void test(String address, String city, String state, String zip, String phone) {
    if (!ValidateInput02.validateAddress(address)) assert false;
    else if (!ValidateInput02.validateCity(city)) System.out.println("Invalid city");
    else if (!ValidateInput02.validateState(state)) System.out.println("Invalid state");
    else if (!ValidateInput02.validateZip(zip)) System.out.println("Invalid zip code");
    else System.out.println("Valid input. Thank you.");
  }
}

public class ValidateInput02 {
  public static boolean validateFirstName(String firstName) {
    return firstName.matches("[A-Z][a-zA-Z]*");
  }

  public static boolean validateLastName(String lastName) {
    return lastName.matches("[a-zA-z]+(['-][a-zA-Z]+)*");
  }

  public static boolean validateAddress(String address) {
    return address.matches("\\d+\\s+([a-zA-Z]+|[a-zA-Z]+\\s[a-zA-Z]+)");
  }

  public static boolean validateCity(String city) {
    return city.matches("([a-zA-Z]+|[a-zA-Z]+\\s[a-zA-Z]+)");
  }

  public static boolean validateState(String state) {
    return state.matches("([a-zA-Z]+|[a-zA-Z]+\\s[a-zA-Z]+)");
  }

  public static boolean validateZip(String zip) {
    return zip.matches("\\d{5}");
  }

  public static boolean validatePhone(String phone) {
    return phone.matches("[1-9]\\d{2}-[1-9]\\d{2}-\\d{4}");
  }
}
