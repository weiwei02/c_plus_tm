//
// Created by wangww on 2020/4/6.
//
#include <iostream>
#include <string>
#include <vector>
using std::string;
struct Review{
    string title;
    int rating;
};

bool FillReview(Review &rr);

void ShowReview(const Review &);
using std::cout;
int t_vector(){

    using std::vector;

    vector<Review> books;
    Review temp;

    while (FillReview(temp)){
        books.push_back(temp);
    }
    int number = books.size();
    if (number > 0){
        cout << "Thank you .You entered the following:\n";
        for (int i = 0; i < number; ++i) {
            ShowReview(books[i]);
        }
        cout << "Reprising: \n" << "Rating\tBook\n";

        vector<Review>::iterator pr;
        for (pr = books.begin(); pr != books.end(); pr++) {
            ShowReview(*pr);
        }

        // copy constructor used
        vector<Review> oldlist(books);
        if (number > 3){
            // remove 2 elements
            books.erase(books.begin() + 1, books.begin() + 3);
            cout << "After erase:\n";
            for (const auto& review : books)
                ShowReview(review);

            // inster 1 item
            books.insert(books.begin(), oldlist.begin() + 1, oldlist.begin() + 2);
            cout << "After insertion:\n";
            for (const auto& review : books)
                ShowReview(review);
        }

        books.swap(oldlist);
        cout << "Swapping oldlist with books\n";
        for (const auto& review : books)
            ShowReview(review);
    } else
        cout << "Nothing entered, nothing gained.\n";
    return 0;
}

bool FillReview(Review &rr){
    cout << "Enter book title (quit to quit):" ;
    std::getline(std::cin, rr.title);
    if (rr.title == "quit")
        return false;
    cout << "Enter book rating: ";
    std::cin >> rr.rating;
    if (!std::cin){
        return false;
    }
    // get rid of input line
    while (std::cin.get() != '\n')
        continue;
    return true;
}

void ShowReview(const Review &rr) {
    cout << rr.rating << "\t" << rr.title << std::endl;
}
