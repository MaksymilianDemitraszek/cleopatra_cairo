func main():
    [ap] = 150; ap++
    call fib_wrapper
    ret
end

func fib_wrapper(n):
    # Call fib(1, 1, 100).
    [ap] = 1; ap++
    [ap] = 1; ap++
    [ap] = 150; ap++
    call fib

    # Make sure the 100th Fibonacci number is 927372692193078999176.
#    [ap - 1] = 927372692193078999176
    [ap - 1] = 26099748102093884802012313146549

    if n != 0:
        [ap] = n - 1; ap++
        call fib_wrapper
    end
    ret
end

func fib(first_element, second_element, n) -> (res : felt):
    jmp fib_body if n != 0
    [ap] = second_element; ap++
    ret

    fib_body:
    [ap] = second_element; ap++
    [ap] = first_element + second_element; ap++
    [ap] = n - 1; ap++
    call fib
    ret
end
