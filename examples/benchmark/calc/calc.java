public class calc {
  
  public static void do_stuff(String a, String b) {
    try {
      int x = Integer.parseInt(a);
      int y = Integer.parseInt(b);
      assert Integer.parseInt(a) != Integer.parseInt(b) || x == y;
    } catch (java.lang.NumberFormatException e) {
    }
  }

  public static void test(int size, String arg1, String arg2) {
    if (size < 2) {
      System.out.println("need two arguments");
      return;
    }
    String[] args = new String[size];
    args[0] = arg1;
    args[1] = arg2;
    do_stuff(args[0], args[1]);
  }
}
