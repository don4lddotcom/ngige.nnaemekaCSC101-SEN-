use std::io;

fn main() {
    let mut siblings_no_input = String::new();
    println!("Enter the number of siblings you have in numbers: ");
    io::stdin().read_line(&mut siblings_no_input).expect("Please enter only the number of siblings!!");
    let siblings_no: i32 = siblings_no_input.trim().parse().expect("INVALID INPUT:\nPlease type in only numbers!");

    for i in 0..siblings_no {
        println!("\nInformation about sibling #{}", i + 1);

        let mut age_input = String::new();
        println!("Enter the sibling's age: ");
        io::stdin().read_line(&mut age_input).expect("Please enter only the age of the sibling!!");
        let siblings_age: i32 = age_input.trim().parse().expect("INVALID INPUT:\nPlease type in only numbers!");

        let mut siblings_firstname = String::new();
        println!("Please enter his/her first name: ");
        io::stdin().read_line(&mut siblings_firstname).expect("Please enter only his/her name!!");
        let name = siblings_firstname.trim();

        let mut siblings_gender = String::new();
        println!("Please enter his/her gender: ");
        io::stdin().read_line(&mut siblings_gender).expect("Please enter only his/her gender!!");
        let gender = siblings_gender.trim();

        let mut siblings_country_of_residence_input = String::new();
        println!("Please enter his/her country of residence: ");
        io::stdin().read_line(&mut siblings_country_of_residence_input).expect("Please enter only his/her country of residence!!");
        let country = siblings_country_of_residence_input.trim();

        if siblings_age >= 18 {
            let mut love_life_input = String::new();
            println!("Is the sibling married, single, or in a relationship?");
            io::stdin().read_line(&mut love_life_input).expect("Please enter marital, relationship, or single status");
            let love_life = love_life_input.trim();

            if love_life == "married" {
                let mut children_number_input = String::new();
                println!("Enter the number of children the sibling has: ");
                io::stdin().read_line(&mut children_number_input).expect("Please enter only the number of children!!");
                let childrens_no: i32 = children_number_input.trim().parse().expect("INVALID INPUT:\nPlease type in only numbers!");

                for j in 0..childrens_no {
                    println!("\nInformation about child #{}", j + 1);

                    let mut childrens_firstname = String::new();
                    println!("Please enter the child's first name: ");
                    io::stdin().read_line(&mut childrens_firstname).expect("Please enter only the child's name!!");
                    let childrens_name = childrens_firstname.trim();

                    let mut childrens_age_input = String::new();
                    println!("Enter the child's age: ");
                    io::stdin().read_line(&mut childrens_age_input).expect("Please enter only the age of the child!!");
                    let childrens_age: i32 = childrens_age_input.trim().parse().expect("INVALID INPUT:\nPlease type in only numbers!");

                    let mut childrens_school_input = String::new();
                    println!("Please enter the child's school name: ");
                    io::stdin().read_line(&mut childrens_school_input).expect("Please enter only the child's school name!!");
                    let childrens_school = childrens_school_input.trim();
                }

                let mut location_of_family = String::new();
                println!("Please enter the location of the family: ");
                io::stdin().read_line(&mut location_of_family).expect("Please enter the location!!");
                let location_of_family_at_the_moment = location_of_family.trim();

                println!("The family currently resides in {}", location_of_family_at_the_moment);
            } else if love_life == "single" {
                let mut employment_status_input = String::new();
                println!("Is the sibling a student or employed?");
                io::stdin().read_line(&mut employment_status_input).expect("Please enter either 'student' or 'employed'!!");
                let employment_status = employment_status_input.trim();

                if employment_status == "student" {
                    let mut university = String::new();
                    println!("Please enter the university: ");
                    io::stdin().read_line(&mut university).expect("Please enter the university name!!");
                    let university = university.trim();

                    let mut course_of_study = String::new();
                    println!("Please enter the course of study: ");
                    io::stdin().read_line(&mut course_of_study).expect("Please enter the course of study!!");
                    let course_of_study = course_of_study.trim();

                    let mut year_of_study = String::new();
                    println!("Please enter the year of study: ");
                    io::stdin().read_line(&mut year_of_study).expect("Please enter the year of study!!");
                    let year_of_study = year_of_study.trim();

                    let mut abroad = String::new();
                    println!("Is the sibling studying abroad? (yes/no): ");
                    io::stdin().read_line(&mut abroad).expect("Please enter 'yes' or 'no'!!");
                    let abroad = abroad.trim();

                    if abroad == "yes" {
                        println!("The sibling is studying in a foreign country.");
                    } else {
                        println!("The sibling is studying in their home country.");
                    }
                } else if employment_status == "employed" {
                    let mut job_type = String::new();
                    println!("Is the job remote, on-site, or hybrid?");
                    io::stdin().read_line(&mut job_type).expect("Please enter the job type!!");
                    let job_type = job_type.trim();

                    if job_type == "on-site" {
                        let mut company_name = String::new();
                        println!("Please enter the company name: ");
                        io::stdin().read_line(&mut company_name).expect("Please enter the company name!!");
                        let company_name = company_name.trim();

                        let mut job_title = String::new();
                        println!("Please enter the job title: ");
                        io::stdin().read_line(&mut job_title).expect("Please enter the job title!!");
                        let job_title = job_title.trim();

                        let mut industry = String::new();
                        println!("Please enter the industry sector: ");
                        io::stdin().read_line(&mut industry).expect("Please enter the industry!!");
                        let industry = industry.trim();
                    }
                }
            } else if love_life == "in a relationship" {
                let mut relationship_duration = String::new();
                println!("Enter the relationship duration in years: ");
                io::stdin().read_line(&mut relationship_duration).expect("Please enter the duration!!");
                let relationship_duration = relationship_duration.trim();

                let mut partner_name = String::new();
                println!("Enter the partner's first name: ");
                io::stdin().read_line(&mut partner_name).expect("Please enter the partner's name!!");
                let partner_name = partner_name.trim();

                let mut living_together = String::new();
                println!("Do they live together? (yes/no): ");
                io::stdin().read_line(&mut living_together).expect("Please enter 'yes' or 'no'!!");
                let living_together = living_together.trim();

                if living_together == "yes" {
                    let mut city = String::new();
                    println!("Enter the city where they reside: ");
                    io::stdin().read_line(&mut city).expect("Please enter the city!!");
                    let city = city.trim();

                    println!("They reside in {}.", city);
                }
            }
        } else {
            let mut waec_status = String::new();
            println!("Has the sibling completed WAEC? (yes/no): ");
            io::stdin().read_line(&mut waec_status).expect("Please enter 'yes' or 'no'!!");
            let waec_status = waec_status.trim();

            if waec_status == "yes" {
                let mut school_name = String::new();
                println!("Enter the secondary school attended: ");
                io::stdin().read_line(&mut school_name).expect("Please enter the school name!!");
                let school_name = school_name.trim();

                let mut grade = String::new();
                println!("Enter the final grade: ");
                io::stdin().read_line(&mut grade).expect("Please enter the grade!!");
                let grade = grade.trim();

                let mut year_of_completion = String::new();
                println!("Enter the year of WAEC completion: ");
                io::stdin().read_line(&mut year_of_completion).expect("Please enter the year!!");
                let year_of_completion = year_of_completion.trim();
            } else {
                let mut current_class = String::new();
                println!("Enter the current class level: ");
                io::stdin().read_line(&mut current_class).expect("Please enter the class!!");
                let current_class = current_class.trim();

                let mut plan_waec = String::new();
                println!("Does the sibling plan to take WAEC soon? (yes/no): ");
                io::stdin().read_line(&mut plan_waec).expect("Please enter 'yes' or 'no'!!");
                let plan_waec = plan_waec.trim();

                if plan_waec == "yes" {
                    let mut planned_year = String::new();
                    println!("Enter the planned year for WAEC: ");
                    io::stdin().read_line(&mut planned_year).expect("Please enter the year!!");
                    let planned_year = planned_year.trim();
                }
            }
        }
    }
}
