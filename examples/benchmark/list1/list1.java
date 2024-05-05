class LinkedListEntry {
  public LinkedListEntry Next;
  public int Value;
}

class LinkedList {
  public LinkedListEntry Head;

  public int size() {
    int count = 0;
    for (LinkedListEntry entry = Head; entry != null; entry = entry.Next) ++count;
    return count;
  }

  public void add(int index, int e) {
    LinkedListEntry newEntry = new LinkedListEntry();
    newEntry.Value = e;
    if (index == 0) {
      newEntry.Next = Head;
      Head = newEntry;
      return;
    }
    LinkedListEntry entry = Head;
    for (int i = 1; i < index; ++i) entry = entry.Next;
    newEntry.Next = entry.Next;
    entry.Next = newEntry;
  }

  public void add(int e) {
    add(size(), e);
  }

  public void remove(int index) {
    if (index == 0) {
      Head = Head.Next;
      return;
    }
    LinkedListEntry entry = Head;
    for (int i = 1; i < index; ++i) entry = entry.Next;
    entry.Next = entry.Next.Next;
  }

  public int get(int index) {
    LinkedListEntry entry = Head;
    for (int i = 0; i < index; ++i) entry = entry.Next;
    return entry.Value;
  }
}

class Utils_synthesis {
  public static int accumulator(int aggregated, int e) {
    if (e % 2 == 0) if (aggregated < e) return e;
    return aggregated;
  }

  public static boolean predicate(int lhs) {
    return true;
  }
}

public class list1 {
  private static int stream(LinkedList list) {
    // java.util.stream.Stream.filter(...)
    int index = 0;
    for (LinkedListEntry entry = list.Head; entry != null; entry = entry.Next) {
      if (Utils_synthesis.predicate(entry.Value)) ++index;
      else list.remove(index);
    }

    // java.util.stream.Stream.reduce(...)
    int aggregated = 0;
    for (LinkedListEntry it = list.Head; it != null; it = it.Next) {
      aggregated = Utils_synthesis.accumulator(aggregated, it.Value);
    }

    return aggregated;
  }

  public static void test(int inputLength, java.util.function.IntSupplier nondetInt) {
    LinkedList lhs = new LinkedList();
    LinkedList rhs = new LinkedList();
    int size = inputLength;
    for (int i = 0; i < size; ++i) {
      int value = nondetInt.getAsInt();
      lhs.add(i, value);
      rhs.add(i, value);
    }

    int lhs_result = 0;
    for (LinkedListEntry it = lhs.Head; it != null; it = it.Next) {
      if (it.Value % 2 == 0) if (lhs_result < it.Value) lhs_result = it.Value;
    }

    int rhs_result = stream(rhs);

    assert (lhs_result == rhs_result);
  }
}
