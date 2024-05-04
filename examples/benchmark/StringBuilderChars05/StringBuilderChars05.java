public class StringBuilderChars05 {
  public static void test(String arg) {
    StringBuilder buffer = new StringBuilder(arg);
    buffer.setCharAt(0, 'H');
    buffer.setCharAt(6, 'T');
    assert buffer.toString().equals("HiffBllTe Limited");
  }
}
