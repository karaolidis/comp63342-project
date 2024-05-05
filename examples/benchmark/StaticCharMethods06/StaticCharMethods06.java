public class StaticCharMethods06 {
  
  public static void test(String arg) {
    if (arg.length() < 1) return;

    char c = arg.charAt(0);
    Character c1 = c;
    Character c2 = c;

    if (c1.equals(c2)) {
      System.out.println("c1 and c2 are equal\n");
      assert true;
    } else {
      System.out.println("c1 and c2 are not equal\n");
      assert false;
    }
  }
}
