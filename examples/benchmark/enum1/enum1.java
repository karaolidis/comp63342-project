enum enum1 {
  VAL1,
  VAL2,
  VAL3,
  VAL4,
  VAL5;

  static {
    for (enum1 e : enum1.values()) {
      System.out.println(e);
    }
  }
}

public class enum1 {
  public static void test(boolean assertion) {
    enum1 e = enum1.VAL5;
    assert (assertion == (e == enum1.VAL5));
  }
}
