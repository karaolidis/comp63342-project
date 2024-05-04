public class StringValueOf05 {
  public static void test(String input) {
    if (input == null || input.length() < 1) return;

    char characterValue = input.charAt(0);
    String tmp = String.valueOf(characterValue);
    assert tmp.equals("A");
  }
}
