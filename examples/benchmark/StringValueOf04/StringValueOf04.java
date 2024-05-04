public class StringValueOf04 {
  public void test(boolean booleanValue) {
    String tmp = String.valueOf(booleanValue);
    assert tmp.equals("true");
  }
}
