public class StringValueOf09 {
  public void test(String arg) {
    double doubleValue = Double.parseDouble(arg); // no suffix, double is default
    String tmp = String.valueOf(doubleValue);
    assert tmp.equals("33.3333");
  }
}
