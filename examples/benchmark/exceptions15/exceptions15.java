class InetAddress {}

class InetSocketAddress {}

public class exceptions15 {
  public static String lookupPtrRecord(InetAddress address) {
    return "Foo";
  }

  public static InetAddress reverse(InetAddress address) {
    return address;
  }

  public static void test(InetAddress address) {
    try {
      String domainName = lookupPtrRecord(reverse(address));
    } catch (Exception e) {
      assert false;
    }
  }
}
