public class StringBuilderConstructors01 {
  public void test(String nondetString) {
    String arg = nondetString;
    if (arg.length() < 1) return;

    StringBuilder buffer1 = new StringBuilder();
    StringBuilder buffer2 = new StringBuilder(10);
    StringBuilder buffer3 = new StringBuilder(arg);

    assert buffer1.length() == 0;
    assert buffer2.length() == 0;
    assert buffer3.length() > 0;
  }
}
