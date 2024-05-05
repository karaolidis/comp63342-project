class iarith2 {
  public static int i() {
    int value = 3;
    value &= 1;
    value |= 4;
    value ^= 4;
    value <<= 2;
    value >>= 1;
    value = -value;
    value >>>= 1;
    return value;
  }

  public static long l() {
    long value = 3L;
    value &= 1L;
    value |= 4L;
    value ^= 4L;
    value <<= 2L;
    value >>= 1L;
    value = -value;
    value >>>= 1L;
    return value;
  }

  public static void s() {
    short lhs = 127;
    short rhs = 32767;
    if (lhs < rhs) {
      return;
    }
    assert false;
  }

  public static void test() {
    assert i() == 2147483647;
    assert l() == 9223372036854775807L;
    s();
  }
}
