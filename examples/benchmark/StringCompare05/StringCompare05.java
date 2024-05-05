public class StringCompare05 {
  public static void test(String string) {
    String s1 = new String(string);
    if (s1 == string) // false; they are not the same object
      assert true;
    else 
      assert false;
  }
}
