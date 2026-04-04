import { Injectable } from '@structa/runtime';
import { User, CreateUserDto, UpdateUserDto } from '../dtos/user.dto';

@Injectable()
export class UserService {
    private users: User[] = [
        { id: '1', name: 'John Doe', email: 'john@example.com', createdAt: new Date() },
        { id: '2', name: 'Jane Doe', email: 'jane@example.com', createdAt: new Date() }
    ];

    async findAll(): Promise<User[]> {
        return this.users;
    }

    async findById(id: string): Promise<User | null> {
        return this.users.find(user => user.id === id) || null;
    }

    async create(data: CreateUserDto): Promise<User> {
        const newUser: User = {
            id: String(this.users.length + 1),
            name: data.name,
            email: data.email,
            createdAt: new Date()
        };
        this.users.push(newUser);
        return newUser;
    }

    async update(id: string, data: UpdateUserDto): Promise<User | null> {
        const index = this.users.findIndex(user => user.id === id);
        if (index === -1) return null;
        
        this.users[index] = { ...this.users[index], ...data };
        return this.users[index];
    }

    async delete(id: string): Promise<boolean> {
        const index = this.users.findIndex(user => user.id === id);
        if (index === -1) return false;
        
        this.users.splice(index, 1);
        return true;
    }
}
