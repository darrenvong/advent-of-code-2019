from collections import Counter

def get_all_possible_passwords(lower_bound, upper_bound):
    return range(lower_bound, upper_bound + 1)

def has_pairs(password):
    password_str = str(password)
    for i in range(5):
        if password_str[i] == password_str[i + 1]: return True
    return False

def has_exact_pairs(password):
    digit_with_pairs = filter(lambda count: count == 2, Counter(str(password)).values())
    return len(list(digit_with_pairs)) > 0

def is_same_or_higher(password):
    password_str = str(password)
    for i in range(len(password_str) - 1):
        if password_str[i + 1] < password_str[i]:
            return False
    return True


if __name__ == "__main__":
    lower_bound, upper_bound = 246515, 739105
    passwords = get_all_possible_passwords(lower_bound, upper_bound)
    
    qualified_passwords = filter(is_same_or_higher, filter(has_pairs, passwords))

    print("=" * 15, "Part 1", "=" * 15)
    print(f"{len(list(qualified_passwords))} passwords meet the criteria.")
    print()

    passwords = get_all_possible_passwords(lower_bound, upper_bound)
    strict_qualified_passwords = filter(is_same_or_higher, filter(has_exact_pairs, passwords))
    print("=" * 15, "Part 2", "=" * 15)
    print(f"{len(list(strict_qualified_passwords))} passwords meet the criteria.")
