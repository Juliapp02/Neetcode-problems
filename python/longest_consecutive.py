
def longestConsecutive(nums: list[int]) -> int:
    # By default, name_variable = {} creates a dictionary.
    # For an empty set ->  name_variable = set()
    nums_set = set(nums)
    counter = 0
    prob_counter = 0

    for i in nums_set:
        if (i-1) not in nums_set:
            actual = i
            prob_counter += 1

            while (actual+1) in nums_set:
                actual += 1
                prob_counter += 1

            if prob_counter > counter:
                counter = prob_counter
            
            prob_counter = 0

    return counter


def main():
    v = [0, 5, 1, 2, 3, 4]
    solution = longestConsecutive(v)
    print(solution) 

    

main()
