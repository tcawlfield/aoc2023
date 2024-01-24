#include <iostream>
#include <string>
#include <sstream>
#include <fstream>
#include <cassert>
#include <cstdint>
#include <iterator>
#include <numeric>
#include <list>
// #include <vector>

#define INPUT "inputs/input_9.txt"
using namespace std;
using Int = int64_t;

namespace day9
{

    class Sequence
    {
    public:
        static Sequence from_str(const string &str)
        {
            Sequence seq;
            stringstream stream(str);
            while (1)
            {
                Int n;
                stream >> n;
                if (!stream)
                    break;
                seq.seq.push_back(n);
            }
            return seq;
        }

        list<Int> seq;

        list<list<Int>> next_deltas()
        {
            list<list<Int>> deltas;
            list<Int> *current_list = &seq;
            while (1)
            {
                list<Int> next_deltas(current_list->size());
                adjacent_difference(current_list->begin(), current_list->end(), next_deltas.begin());
                next_deltas.pop_front();
                // for (auto const &v : next_deltas)
                //     cout << v << " ";
                // cout << endl;
                deltas.push_back(next_deltas);
                bool all_zero = true;
                for (auto &dx : next_deltas)
                {
                    if (dx != 0)
                    {
                        all_zero = false;
                        break;
                    }
                }
                if (all_zero)
                {
                    break;
                }
                current_list = &deltas.back();
            }
            return deltas;
        }

        Int next()
        {
            auto deltas = next_deltas();
            Int dx = 0;
            for (auto diffs = deltas.rbegin(); diffs != deltas.rend(); diffs++)
            {
                // for (auto const &v : *diffs)
                //     cout << v << " ";
                // cout << endl;
                dx += diffs->back();
            }
            return seq.back() + dx;
        }
    };

    int main()
    {
        auto ex1 = Sequence::from_str("0   3   6   9  12  15").next();
        assert(ex1 == 18);
        assert(Sequence::from_str("1 3 6 10 15 21").next() == 28);
        assert(Sequence::from_str("10  13  16  21  30  45").next() == 68);

        ifstream file(INPUT);
        string line;
        Int ttl = 0;
        while (getline(file, line))
        {
            Sequence s = Sequence::from_str(line);
            ttl += s.next();
        }
        cout << "Part 1: " << ttl << endl;

        return 0;
    }

}