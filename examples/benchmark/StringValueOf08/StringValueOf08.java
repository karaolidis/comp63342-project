public class StringValueOf08 {
  public static void test(String arg) {
    float floatValue = Float.parseFloat(arg);
    String tmp = String.valueOf(floatValue);
    assert tmp.equals("2.50");
  }
}
