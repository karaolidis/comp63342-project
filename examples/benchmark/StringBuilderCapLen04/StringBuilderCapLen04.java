public class StringBuilderCapLen04 {
  public void test(String input) {
    StringBuilder buffer = new StringBuilder(input);
    assert buffer.capacity() == 69;
  }
}
