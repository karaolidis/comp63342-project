public class StringValueOf09 {
  public static void test(String nondetString) {
    String arg = nondetString;

    double doubleValue = Double.parseDouble(arg); // no suffix, double is default
    String tmp = String.valueOf(doubleValue);
    assert tmp.equals("33.3333");
  }
}
