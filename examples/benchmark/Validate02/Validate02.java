public class Validate02 {
  public static void test(String address, String city, String state, String zip, String phone) {
    if (!ValidateInput02.validateAddress(address)) {
      throw new AssertionError("Address validation failed.");
    } else if (!ValidateInput02.validateCity(city)) {
      System.out.println("Invalid city");
    } else if (!ValidateInput02.validateState(state)) {
      System.out.println("Invalid state");
    } else if (!ValidateInput02.validateZip(zip)) {
      System.out.println("Invalid zip code");
    } else {
      System.out.println("Valid input.  Thank you.");
    }
  }
}
